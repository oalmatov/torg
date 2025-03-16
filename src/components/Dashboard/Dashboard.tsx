import './Dashboard.css';
import { createContext, useState, Dispatch } from 'react';
import PfpIcon from '../../assets/icons/profile-icon.png';

import { Community, User, Chat } from '../../types';

import Navbar from "./Navbar";
import UserView from './UserView';
import ChatView from './ChatView';
import CommunityView from './CommunityView';

const Shevchenko146: Community = {
    id: 1,
    name: "Shevchenko 146",
    members: [],
};

const CuriousSouls: Community = {
    id: 2,
    name: "Curious Souls",
    members: [],
};

const Outsiders: Community = {
    id: 3,
    name: "Outsiders",
    members: [],
};

const torgin: User = {
    id: 3,
    name: 'torgin',
    picture: PfpIcon,
    communities: [Shevchenko146, CuriousSouls, Outsiders],
    services: [],
    chats: [
        {id: 1, name: 'anuar', messages: []},
        {id: 2, name: 'will', messages: []},
        {id: 2, name: 'omar', messages: []},
    ]
}

interface Tab {
    type: 'user' | 'community' | 'chat';
    data: User | Community | Chat;
}

interface DashboardContextType {
    user: User;
    activeTab: Tab;
    setActiveTab: Dispatch<Tab> | null;
}


export const DashBoardContext = createContext<DashboardContextType>({
    user: torgin,
    activeTab: {type: 'user', data: torgin},
    setActiveTab: null,
});


function Dashboard() {
    const [activeTab, setActiveTab] = useState<Tab>({type: 'user', data: torgin});

    const viewMap = {
        'user': <UserView/>,
        'chat': <ChatView/>,
        'community': <CommunityView/>,
    }

    return (
        <DashBoardContext value={{user: torgin, activeTab: activeTab, setActiveTab: setActiveTab}}>
            <div className="app-container">
                <Navbar/>
                {viewMap[activeTab.type]}
            </div>
        </DashBoardContext>
    )
}

export default Dashboard;
