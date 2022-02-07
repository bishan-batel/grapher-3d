import React, {FC, useEffect, useRef, useState} from "react";
import {BrowserRouter, Route, Routes} from "react-router-dom";
import GraphPage from "./components/graph/GraphPage";
import AccountPage from "./components/login/AccountPage";

const getStoredTheme = () => {
    const stored = localStorage.getItem("theme");
    if (stored === "dark") return "dark";
    if (stored === "light") return "light";
    return "dark"
}

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

    const [theme, setTheme] = useState<"dark" | "light">(getStoredTheme());

    const toggleTheme = () => {
        const newTheme = theme === "dark" ? "light" : "dark";
        localStorage.setItem("theme", newTheme);
        setTheme(newTheme)
    };

    return (
        <div className={"App " + theme}>
            {hasGl ?
                <BrowserRouter>
                    <Routes>
                        {/*Graph Page Routes*/}
                        <Route path="" element={<GraphPage/>}/>
                        <Route path="graphing" element={<GraphPage/>}/>
                        <Route path="account" element={<AccountPage/>}/>
                    </Routes>
                </BrowserRouter> :
                <NoGl msg={err}/>}
            <img src="icons/brush.svg" id="theme-switcher" onClick={toggleTheme}/>
        </div>
    );
};

const NoGl: FC<{ msg: string }> = ({msg}) => (<>
    <h1 id="no-gl">{msg}</h1>
</>);

const hasWebGL2 = (): [boolean, string] => {
    const gl = document.createElement('canvas').getContext('webgl2');
    if (!gl) {
        if (typeof WebGL2RenderingContext !== 'undefined') {
            return [false, 'Your browser appears to support WebGL2 but it might be disabled. Try updating your OS and/or video card drivers'];
        } else {
            return [false, 'Your browser has no WebGL2 support at all'];
        }
    }
    return [true, ""];
}

export default App;
