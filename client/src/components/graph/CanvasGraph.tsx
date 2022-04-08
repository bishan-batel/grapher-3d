import {FC, useContext, useEffect, useRef, useState} from "react";
import {CrateContext, GraphDataContext} from "./GraphPage";
import {Crate} from "../../utils/hooks";
import {GlobalGrapher} from "../../crate-build";
import {GraphData} from "../../types/graph";
import graphSidePanel from "./GraphSidePanel";

const PAN_SENSITIVITY = 0.01;
const ZOOM_SENSITIVITY = 0.01;
const SMOOTHING = 0.1;
const MIN_ZOOM = 0.0001;

const CanvasGraph: FC = () => {
    const canvasRef = useRef<any>();
    const crate = useContext<Crate | null>(CrateContext);
    const [graph] = useContext(GraphDataContext);
    const [grapher, setGrapher] = useState<GlobalGrapher | null>(null);

    // run once
    useEffect(() => {
        if (crate === null) return;

        // gets canvas element
        const canvas = canvasRef.current as HTMLCanvasElement;

        // initialize grapher program
        let grapher: GlobalGrapher;
        try {
            grapher = crate.canvas_init(`#${canvas.id}`) as GlobalGrapher;
        } catch (err) {
            window.alert(err);
            return;
        }

        // frees memory when window is unloaded
        window.onunload = window.onbeforeunload = () => grapher.free();

        // initializes grapher canvas
        grapher.init();

        // input handlers to control camera & send data to grapher

        // is mouse up/down
        let mouseDown = false;
        canvas.onmousedown = () => mouseDown = true;
        canvas.onmouseup = () => mouseDown = false;

        // sets camera rotation whenever mouse moves
        const cam = {
            rotX: Math.PI / -8,
            rotY: Math.PI / 5,
            zoom: 1.
        }
        canvas.onmousemove = (ev) => {
            if (!mouseDown) return;

            cam.rotX = grapher.cam_rot_x() + -ev.movementY * PAN_SENSITIVITY;
            cam.rotY = grapher.cam_rot_y() + -ev.movementX * PAN_SENSITIVITY;
        }

        canvas.onwheel = (ev) => {
            cam.zoom = Math.abs(grapher.cam_zoom() + ev.deltaY * ZOOM_SENSITIVITY) + MIN_ZOOM;
        }

        const lerp = (from: number, to: number, a: number) => from + (to - from) * a;

        // triggers infinite render loop
        (function loop() {
            grapher.render();

            // lerps values to create smoothing effect
            grapher.set_cam_rot(
                lerp(grapher.cam_rot_x(), cam.rotX, SMOOTHING),
                lerp(grapher.cam_rot_y(), cam.rotY, SMOOTHING),
            );

            grapher.set_cam_zoom(lerp(grapher.cam_zoom(), cam.zoom, SMOOTHING));
            requestAnimationFrame(loop);
        })();

        // resizes canvas based on window
        window.onresize = () => {
            canvas.width = window.innerWidth;
            canvas.height = window.innerHeight;
            grapher.set_viewport();
        };

        
        setGrapher(grapher);

        // Effect depends on canvas element as well as WASM crate
    }, [crate, canvasRef]);

    const handleReload = () => {
        if (!(crate !== null && graph !== null && grapher !== null)) return;
        // maps equations to an array to make it easy for rust to parse it
        const equations = graph.equations.map(equation => [equation.disabled, equation.equation]);

        try {
            grapher.set_animate(graph.animate);
            grapher.set_equations(equations);
        } catch (err) {
            window.alert(err);
        }
    }

    return <>
        <canvas id="graph-canvas" width={window.innerWidth} height={window.innerHeight} ref={canvasRef}/>
        <button id="graph-reload" onClick={handleReload}>Reload</button>
    </>;
}

export default CanvasGraph;
