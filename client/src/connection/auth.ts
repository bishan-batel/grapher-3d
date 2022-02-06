// TODO document

import {useEffect, useState} from "react";
import {authFetch} from "./conn";

let globalUser: User | null = null;

// Retrieves session token from browser cookies
export const sessionTok = (): string => getCookie("token") as string;

const getCookie = (name: string): string | null => {
    const nameEQ = name + "=";
    const cookies = document.cookie.split(';');
    for (let i = 0; i < cookies.length; i++) {
        let cookie = cookies[i];
        while (cookie.charAt(0) === ' ') cookie = cookie.substring(1, cookie.length);
        if (cookie.indexOf(nameEQ) === 0) return cookie.substring(nameEQ.length, cookie.length);
    }
    return null;
}

// Sends request to log in
export const login = async (email: string, password: string): Promise<string> => {
    // TODO login function
    const response = await authFetch("login", {
        method: "post",
        args: {
            email,
            password
        }
    });

    switch (response.status) {
        case 200:
            let tok = await response.text();
            document.cookie = "token=" + tok;
            console.log(sessionTok())
            dispatchAuthChange({email});
            return "Successful"
        case 401:
            return "Invalid Password";
        case 404:
            return "User does not exist";
        case 500:
            return "Internal Error"
        default:
            return `Unknown Error Occurred with Status Code ${response.status}`
    }
}


export const register = async (email: string, password: string): Promise<string> => {
    // TODO login function
    const response = await authFetch("register", {
        method: "post",
        args: {
            email,
            password
        }
    });

    switch (response.status) {
        case 200:
            let tok = await response.text();
            document.cookie = "token=" + tok;
            console.log(sessionTok())
            dispatchAuthChange({email});
            return "Successful"
        case 409:
            throw new Error("Duplicate username")
        case 500:
            throw new Error("Internal Error")
        default:
            throw new Error(`Unknown Error Occurred with Status Code ${response.status}`)
    }
}

export const validateLogin = async () => {
    const response = await authFetch("validate");


    switch (response.status) {
        case 200:
            const user = await response.json() as User;
            dispatchAuthChange(user);
            return;
        case 404:
            console.log("Failed to validate");
            dispatchAuthChange(null);
            return;
        default:
            throw new Error(`Unknown status code ${response.status}`);
    }
}

// user data structure
export interface User {
    email: string
}

export const useAuth = () => {
    const [user, setUser] = useState<User | null>(globalUser);

    // run once
    useEffect(() => {

        // on auth change
        window.addEventListener("authChange", (ev) => {

            // set user to current user
            const info = (ev as CustomEvent<AuthChange>).detail;
            setUser(info.user);
        });
    }, []);

    return user;
}

export const dispatchAuthChange = (user: User | null) => {
    globalUser = user;
    window.dispatchEvent(
        new CustomEvent<AuthChange>("authChange", {detail: {user}})
    );
}

interface AuthChange {
    user: User | null,
}
