import React, {
    ChangeEventHandler,
    createContext, Dispatch,
    FC, KeyboardEventHandler, SetStateAction,
    useContext,
    useEffect,
    useRef,
    useState
} from "react";
import "./GraphPage.scss";
import "./GraphSelect.scss";
import {EquationData, GraphData} from "../../types/graph";
import CanvasGraph from "./CanvasGraph";
import {useCrate, Crate} from "../../utils/hooks";
import GraphSidePanel from "./GraphSidePanel";
import {useAuth, validateLogin} from "../../connection/auth";
import {createGraph, retrieveGraph, retrieveGraphNames} from "../../connection/db";
import GraphSelectScreen from "./GrapahSelect";

// TODO document
const RESTRICTED_CHARACTERS = "\b\n\"':".split("");

const EquationDropdown: FC<{ z: number }> = ({z}) => {
    const [graph, setGraph] = useContext(GraphDataContext);
    if (graph === null) return <></>;

    // toggles disabled
    const handleVisibleClick = () => {
        try {
            const newGraph = {...graph};
            newGraph.equations[z].disabled = !graph.equations[z].disabled;
            setGraph(newGraph);
        } catch {
        }
    };

    const handleDelete = () => {
        const newGraph = {...graph};
        // removes Z index
        newGraph.equations.splice(z, 1);
        newGraph.equations.forEach((eq, i) => eq.zIndex = i);
        setGraph(newGraph);
    }

    if (z >= graph.equations.length) return <></>;

    return (
        <span className="dropdown">
            {/*shows eye or closed eye depending on disabled state */}
            <img
                src={graph.equations[z].disabled ? "icons/eye-crossed.svg" : "icons/eye.svg"}
                alt={"Visible"}
                className="is-visible"
                onClick={handleVisibleClick}
            />

            <img
                src="icons/trash.svg"
                className="delete"
                onClick={handleDelete}
            />
        </span>
    );
}

interface EquationPromptProps {
    equation: EquationData;
}


const EquationPrompt: FC<EquationPromptProps> = ({equation}) => {
    const [graph, setGraph] = useContext(GraphDataContext);

    if (graph === null) return <></>

    const onInputKey: KeyboardEventHandler = (ev) => {
        // doesnt allow key to be typed if it is in restricted chars list
        if (RESTRICTED_CHARACTERS.includes(ev.key))
            ev.preventDefault();
    }

    const onTextChange: ChangeEventHandler<HTMLInputElement> = (ev) => {
        const newGraph = {...graph};
        newGraph.equations[equation.zIndex].equation = ev.currentTarget.value;
        setGraph(newGraph);
    }

    let classes = "prompt ";
    if (equation.disabled) classes += "hidden";

    // noinspection HtmlUnknownTarget
    return (
        <li className={classes}>
            <EquationDropdown z={equation.zIndex}/>
            <img src="icons/settings.svg" alt="Config" className="settings-icon"/>
            <input onChange={onTextChange} onKeyDown={onInputKey} value={equation.equation}/>
        </li>
    )
}

const EquationsPanel: FC = () => {
    const [graph, setGraph] = useContext(GraphDataContext);
    const animate = graph?.animate;
    const [visible, setVisible] = useState(true);

    if (graph === null) return <></>

    // Handles
    const toggleAnimate = () => {
        const newGraph = {...graph};
        newGraph.animate = !animate;
        setGraph(newGraph);
    };
    const handleNewGraph = () => {
        const newGraph = {...graph};
        newGraph.equations.push({
            zIndex: graph.equations.length,
            equation: "",
            disabled: false
        });

        setGraph(newGraph);
    }
    const handleVisibleToggle = () => {
        setVisible(!visible);
        console.log(visible);
    }

    // maps all equations to an equation prompt
    const equations = graph
        ?.equations
        .map(eq => <EquationPrompt equation={eq} key={eq.zIndex}/>);

    // noinspection HtmlUnknownTarget
    return (
        <>
            <button id="sidebar-visible">
                <img
                    src={visible ? "icons/eye.svg" : "icons/eye-crossed.svg"}
                    alt={"Visible"}
                    onClick={handleVisibleToggle}
                />
            </button>

            <div className={"equations-panel" + (visible ? "" : " hide")}>
                <ul className={"equations"}>
                    {equations}
                </ul>
                <div className="buttons">
                    <button id="new-graph" onClick={handleNewGraph}>
                        <img src="icons/add.svg" alt="" id="add-icon"/>
                    </button>
                    {/* Animate button, adds on or off class depending on the state */}
                    <button id="animate-button" onClick={toggleAnimate} className={animate ? "on" : "off"}>
                        <img src="icons/pencil.svg" alt="" id="pencil-icon"/>
                        Animate
                    </button>
                </div>
            </div>
        </>
    );
};

export const GraphDataContext = createContext<[GraphData | null, Dispatch<SetStateAction<null | GraphData>>]>(
    [null, () => {
    }]
);

export const CrateContext = createContext<Crate | null>(null);


export const Loading = () => (
    <div className="loading">
        <h1 className="msg">Loading
            <ul className="dots">
                <li className="dot">.</li>
                <li className="dot">.</li>
                <li className="dot">.</li>
            </ul>
        </h1>
        <img
            src="https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fimg.nordangliaeducation.com%2Fresources%2Fus%2F_filecache%2F88a%2F2ae%2F21982-cropped-w220-h240-of-1-FFFFFF-ansari-mansoor-adult-1677.jpg&f=1&nofb=1"
            alt=""
            className="icon"
        />
    </div>
);

const GraphPage = () => {
    // Loads rust crate
    const crate = useCrate();
    const auth = useAuth();
    let [graph, setGraph] = useState<null | GraphData>(null);
    console.log("Graph state updated");

    useEffect(() => {
        validateLogin()
            .catch(console.error)
            .then(() => console.log("Validated Account"));
    }, []);

    // If crate hasn't loaded then display loading screen
    if (crate === null) return (
        <div className="graph-page ">
            <Loading/>
        </div>
    );

    // return <GraphSelectScreen setGraph={setGraph}/>

    if (graph === null && auth != null)
        return <GraphSelectScreen setGraph={setGraph}/>

    graph ??= {
        name: "grapher-3d",
        equations: [{
            zIndex: 0,
            equation: "f(x)=x",
            disabled: false
        }],
        description: "",
        animate: false
    };

    // Normal graph page if WASM binary is loaded
    return (
        <CrateContext.Provider value={crate}>
            <GraphDataContext.Provider value={[graph, setGraph]}>
                <div className="graph-page">
                    <CanvasGraph/>
                    <GraphSidePanel/>
                    <EquationsPanel/>
                    <h1 id="graph-name" onClick={() => setGraph(null)}>{graph.name}</h1>
                </div>
            </GraphDataContext.Provider>
        </CrateContext.Provider>
    )
}

export default GraphPage;