import React, {Dispatch, FC, SetStateAction, useEffect, useState} from "react";
import {GraphData} from "../../types/graph";
import {useAuth} from "../../connection/auth";
import {createGraph, deleteGraph, retrieveGraph, retrieveGraphNames} from "../../connection/db";
import {Loading} from "./GraphPage";

const GraphSelectScreen: FC<{ setGraph: Dispatch<SetStateAction<null | GraphData>> }> = ({setGraph}) => {
    const [graphs, setGraphs] = useState<string[] | undefined>([]);
    const auth = useAuth();

    const refreshGraphs = async () => {
        setGraphs(await retrieveGraphNames());
    }

    useEffect(() => {
        refreshGraphs().catch(console.error);
    }, []);

    // if graphs havent been loaded yet show loading page
    if (graphs == null) return (
        <div className="graph-page">
            <Loading/>
        </div>
    );

    const handleNew = async () => {
        try {
            // creates new graph
            await createGraph(window.prompt("Enter Graph Name:") ?? "Default");

            // refreshes list
            await refreshGraphs();
        } catch (err) {
            // alerts if error
            window.alert(err);
        }
    }

    const select = async (name: string) => {
        try {
            // retrieves graph from database
            const graph = await retrieveGraph(name);

            // update graph page to be selected
            setGraph(graph);
        } catch (err) {
            console.error(err);
        }
    };

    const handleDelete = async (name: string) => {
        try {
            // creates new graph
            await deleteGraph(name);

            // refreshes list
            await refreshGraphs();
        } catch (err) {
            // alerts if error
            window.alert(err);
        }
    };

    const graphsJSX = graphs.map(graph => {
        return (
            <li className="graph">
                <img
                    src="icons/trash.svg"
                    className="delete"
                    onClick={() => handleDelete(graph)}
                />
                <span onClick={() => select(graph)}>{graph}</span>
            </li>
        )
    });

    return (
        <div className="graph-page">
            <h1 id="title">{auth?.email ?? "email@mail.com"}'s Graphs</h1>
            <div className="graph-select">
                <ul id="graphs">
                    {graphsJSX}
                    <button id="create-new" onClick={handleNew}>New</button>
                </ul>
            </div>
        </div>
    );
}

export default GraphSelectScreen;