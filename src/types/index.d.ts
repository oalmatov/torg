export interface Chat {
    id: number;
    user: User;
    messages: string[]
}

export interface Community {
    id: number;
    name: string;
    members: User[];
}

export interface User {
    id: number;
    name: string;
    picture: string | undefined;
    communities: Community[];
    services: string[];
    chats: Chat[];
}
