interface Chat {
    id: number;
    user: User;
    messages: string[]
};

interface Community {
    id: number;
    name: string;
    members: User[];
};

interface User {
    id: number;
    name: string;
    picture: string | null;
    communities: Community[];
    services: string[];
    chats: Chat[];
};

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
    picture: '/assets/icons/profile-icon.png',
    communities: [Shevchenko146],
    services: [],
    chats: [],
}

const will: User = {
    id: 2,
    name: 'will',
    picture: '/assets/icons/profile-icon.png',
    communities: [CuriousSouls],
    services: [],
    chats: [],
}

const omar: User = {
    id: 2,
    name: 'omar',
    picture: '/assets/icons/profile-icon.png',
    communities: [CuriousSouls],
    services: [],
    chats: [],
}

const torgin: User = {
    id: 3,
    name: 'torgin',
    picture: '/assets/icons/profile-icon.png',
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

const navFooter = document.getElementById("nav-footer");

const navFooterContent = `
    <div class="user">
        <img src="${torgin.picture}" alt="Profile picture" class="profile-icon"/>
        <p>${torgin.name}</p>
    </div>
    ${navFooter?.innerHTML}
`;

if (navFooter) {
    navFooter.innerHTML = navFooterContent;
}

const torginCommunitiesHTML = torgin.communities.map((c) => {
    return `<p>${c.name}</p>`;
})

const navCommunities = document.getElementById("nav-communities-container");

const navCommunitiesContent = `
    ${navCommunities?.innerHTML}
    <div class="content">
        ${torginCommunitiesHTML.join('')}
    </div>
`;

if (navCommunities) {
    navCommunities.innerHTML = navCommunitiesContent;
}


const torginChatsHTML = torgin.chats.map((c) => {
    return `
        <div class="user">
            <img src="${c.user.picture}" alt="Profile picture" class="profile-icon"/>
            <p>${c.user.name}</p>
        </div>
    `
});

const navMessages = document.getElementById("nav-messages-container");

const navMessagesContent = `
    ${navMessages?.innerHTML}
    <div class="content">
        ${torginChatsHTML.join('')}
    </div>
`;

if (navMessages) {
    navMessages.innerHTML = navMessagesContent;
}

navCommunities?.addEventListener('click', () => {
    const content = navCommunities.querySelector('.content');
    content?.classList.toggle("hidden");
})

navMessages?.addEventListener('click', () => {
    const content = navMessages.querySelector('.content');
    content?.classList.toggle("hidden");
})
