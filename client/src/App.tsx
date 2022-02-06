import React, {FC, useEffect, useRef} from "react";
import {BrowserRouter, Route, Routes} from "react-router-dom";
import GraphPage from "./components/graph/GraphPage";
import AccountPage from "./components/login/AccountPage";

/// Main Monolithic React Component
const App: FC = () => {
    // run code only once
    useEffect(() => {

        // Creates theme function  for Rust to get theme colors
        const app = document.querySelector(".App") as HTMLDivElement;

        (window as any).theme = (id: number): [number, number, number] => {
            const hex = getComputedStyle(app).getPropertyValue(`--color${id}`);

            // tries to get color and parse hex to RGB
            try {
                const r = parseInt(hex.slice(1, 3), 16) / 255.;
                const g = parseInt(hex.slice(3, 5), 16) / 255.;
                const b = parseInt(hex.slice(5, 7), 16) / 255.;
                return [r, g, b];
            } catch (err) {
                // return white if failed
                return [1., 1., 1.];
            }
        };
    }, []);

    const [hasGl, err] = hasWebGL2();
    return (
        <div className="App dark">
            {hasGl ?
                <BrowserRouter>
                    <Routes>
                        {/*Graph Page Routes*/}
                        <Route path="" element={<GraphPage/>}/>
                        <Route path="graphing" element={<GraphPage/>}/>
                        <Route path="account" element={<AccountPage/>}/>
                    </Routes>
                </BrowserRouter> : <NoGl msg={err}/>}
        </div>
    );
};

const NoGl: FC<{ msg: string }> = ({msg}) => (<>
    <h1 id="no-gl">{msg}</h1>
</>);

const hasWebGL2 = (): [boolean, string] => {
    const gl = document.createElement('canvas').getContext('webgl2');
    if (!gl ) {
        if (typeof WebGL2RenderingContext !== 'undefined') {
            return [false, 'Your browser appears to support WebGL2 but it might be disabled. Try updating your OS and/or video card drivers'];
        } else {
            return [false, 'Your browser has no WebGL2 support at all'];
        }
    }
    return [true, ""];
}

export default App;
