import { useContext } from "react";
import { DashBoardContext } from "./Dashboard";

function UserView() {
    const context = useContext(DashBoardContext);
    return (
        <div className="view-container">
            <h1>{context.activeTab.data.name}</h1>
        </div>
    )
}

export default UserView;
