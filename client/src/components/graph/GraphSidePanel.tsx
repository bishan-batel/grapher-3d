import {FC, useContext, useState} from "react";
import {useAuth} from "../../connection/auth";

import {GraphDataContext} from "./GraphPage";
import {saveGraph} from "../../connection/db";

const SaveButton: FC = () => {
    const auth = useAuth();
    const [graph] = useContext(GraphDataContext);

    // should never be true but this is for worst case scenario
    if (auth === null || graph === null) return <></>

    const handleSave = async () => {
        try {
            await saveGraph(graph);
            // alerts if done successfully
            window.alert(`Graph '${graph.name} saved`);
        } catch (err) {
            // alerts if error
            window.alert(err);
        }
    }

    return (
        <span className="save" onClick={handleSave}>
            <img src="icons/disk.svg" alt="" className="icon"/>
            Save
        </span>
    )
}

const AccountButton: FC = () => {
    const user = useAuth();
    const name = user === null ? "Account" : user.email.split("@")[0];

    // Redirects on click
    const handleClick = () => window.location.replace('/account');

    return (
        <span className="account" onClick={handleClick}>
            <img src="icons/portrait.svg" alt="" className="icon"/>
            {name}
        </span>
    )
}

const GraphSidePanel: FC = () => (
    <div className="side-panel">
        <AccountButton/>
        <SaveButton/>
    </div>
)

export default GraphSidePanel;
