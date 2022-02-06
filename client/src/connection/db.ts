import {GraphData} from "../types/graph";
import {apiFetch} from "./conn";
import {User} from "./auth";

export const deleteGraph = async (name: string): Promise<void> => {
    const response = await apiFetch("delete", {
        method: "post",
        args: {
            name
        },
    });

    // maps status code to response
    switch (response.status) {
        case 200:
            return;
        // Error codes
        case 401:
            throw new Error("You have been denied permission");
        case 500:
            throw new Error("Internal Server Error");
        default:
            throw new Error(`Unknown status code ${response.status}`);
    }
}
export const createGraph = async (name: string): Promise<void> => {
    const response = await apiFetch("create", {
        method: "post",
        args: {
            name
        },
    });

    // maps status code to response
    switch (response.status) {
        case 200:
            return;
        // Error codes
        case 401:
            throw new Error("You have been denied permission");
        case 409:
            throw new Error("Duplicate Graph Name");
        case 500:
            throw new Error("Internal Server Error");
        default:
            throw new Error(`Unknown status code ${response.status}`);
    }
}

export const retrieveGraph = async (name: string): Promise<GraphData> => {
    const response = await apiFetch("req", {
        method: "post",
        args: {
            name
        },
    });

    // maps status code to response
    switch (response.status) {
        case 200:
            // parses request as JSON
            return await response.json();
        // Error Codes
        case 401:
            throw new Error("You have been denied permission");
        case 500:
            throw new Error("Internal Server Error");
        default:
            throw new Error(`Unknown status code ${response.status}`);
    }
}

export const retrieveGraphNames = async (): Promise<string[]> => {
    const response = await apiFetch("graphs");

    // maps status code to response
    switch (response.status) {
        case 200:
            // parses request as JSON
            return (await response.json()).graphs;

        // Error Codes
        case 401:
            throw new Error("You have been denied permission");
        case 500:
            throw new Error("Internal Server Error");
        default:
            throw new Error(`Unknown status code ${response.status}`);
    }

}

export const saveGraph = async (graph: GraphData): Promise<void> => {
    const args: any = {
        "name": graph.name,
        "description": graph.description,
        "animate": graph.animate,
        "equation_length": graph.equations.length
    };

    // maps equation data to format for server
    graph.equations.forEach(equation => {
        args[`${equation.zIndex}_equation`] = equation.equation;
        args[`${equation.zIndex}_disabled`] = equation.disabled;
    });

    const response = await apiFetch("update", {
        method: "post",
        args
    });

    // maps status code from response
    switch (response.status) {
        // HTTP OK
        case 200:
            return;
        // Error Codes
        case 400:
            throw new Error("Bad Request");
        case 401:
            throw new Error("You have been denied permission");
        case 500:
            throw new Error("Internal Server Error");
        default:
            throw new Error(`Unknown error with status code ${response.status}`);
    }
}