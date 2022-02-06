/**
 * WORK IN PROGRESS
 */
import {sessionTok} from "./auth";

interface RequestArgs {
    method: "post" | "put" | "get" | "delete";
    args: any,
}

export const serverFetch = async (
    path: string, reqArgs?: RequestArgs
): Promise<Response> => {
    // creates route to send HTTP request
    const route = `/${path}`;

    // builds body response im YAML format
    let body = "";

    Object.keys(reqArgs?.args ?? {}).forEach(key => {
        body += `${key}: ${reqArgs?.args[key]}\n`;
    });

    // fetch args
    const reqInit: RequestInit = {
        // HTTP verb to send to server
        method: reqArgs?.method ?? "get",
        headers: {
            'Content-Type': 'application/yaml',
            'Cookie': sessionTok()
        },
        cache: "no-cache",
    }

    // if body is not empty send it
    if (body.trim() !== "")
        reqInit.body = body;

    return await fetch(route, reqInit);
}

export const apiFetch = async (path: string, args?: RequestArgs) => await serverFetch(`api/${path}`, args);
export const authFetch = async (path: string, args?: RequestArgs) => await serverFetch(`auth/${path}`, args);
