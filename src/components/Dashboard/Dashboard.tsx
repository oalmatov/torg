import './Dashboard.css';
import PfpIcon from '../../assets/icons/profile-icon.png';

import { Community, User } from '../../types';

import Navbar from "./Navbar";

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

const anuar: User = {
    id: 1,
    name: 'anuar',
    picture: PfpIcon,
    communities: [Shevchenko146],
    services: [],
    chats: [],
}

const will: User = {
    id: 2,
    name: 'will',
    picture: PfpIcon,
    communities: [CuriousSouls],
    services: [],
    chats: [],
}

const omar: User = {
    id: 2,
    name: 'omar',
    picture: PfpIcon,
    communities: [CuriousSouls],
    services: [],
    chats: [],
}

const torgin: User = {
    id: 3,
    name: 'torgin',
    picture: PfpIcon,
    communities: [Shevchenko146, CuriousSouls, Outsiders, Shevchenko146, CuriousSouls, Outsiders],
    services: [],
    chats: [
        {id: 1, user: anuar, messages: []},
        {id: 2, user: will, messages: []},
        {id: 2, user: omar, messages: []},
        {id: 2, user: omar, messages: []},
        {id: 2, user: omar, messages: []},
        {id: 2, user: omar, messages: []},
    ]
}


function Dashboard() {
    return (
        <div className="app-container">
            <Navbar user={torgin} />
            <div className="content"></div>
        </div>
    )
}

export default Dashboard;
