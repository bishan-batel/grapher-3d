import {FC, FormEventHandler, useRef, useState} from "react";
import "./AccountPage.scss";
import {login, register} from "../../connection/auth";

const ReturnLink: FC = () => {
    const handleClick = () => {
        window.location.replace('/');
    }
    return (
        <span id={"return"} onClick={handleClick}>
            Graph
        </span>
    )
}

const Login: FC = () => {
    const emailRef = useRef<any>();
    const passRef = useRef<any>();
    const [mode, setMode] = useState<"login" | "register">("login");

    const handleSubmit: FormEventHandler = async (ev) => {
        ev.preventDefault();

        const email = emailRef.current.value;
        const password = passRef.current.value;

        if (mode === "login") {
            const result = await login(email, password);

            if (result === "Successful") {
                window.alert("Logged In");
                window.location.replace("/");
            } else {
                window.alert(result);
            }
        } else if (mode === "register") {
            try {
                console.log(await register(email, password));
            } catch (err) {
                window.alert(err);
                return;
            }

            const result = await login(email, password);
            if (result === "Successful") {
                window.alert("Registered and Signed In!");
                window.location.replace("/");
            } else {
                window.alert(result);
            }
        }

    }

    const handleClick = () => {
        setMode(mode === "login" ? "register" : "login");
    }

    return (
        <div id="login">
            <div className={"header " + mode} onClick={handleClick}>
                <span className="descrip">
                    Account
                </span>
                <span className="mode">
                    {mode === "login" ? "Login" : "Register"}
                </span>
            </div>

            <form onSubmit={handleSubmit}>
                <span className='email'>
                    <span className='label'>Email:</span>
                    <input type="email" ref={emailRef}/>
                </span>

                <span className='password'>
                    <span className='label'>Password:</span>
                    <input type="password" ref={passRef}/>
                </span>

                <input className="submit" type="submit" value="Submit"/>
            </form>
        </div>
    )
}

const AccountPage: FC = () => {
    return (
        <>
            <ReturnLink/>
            <Login/>
        </>
    )
}

export default AccountPage;