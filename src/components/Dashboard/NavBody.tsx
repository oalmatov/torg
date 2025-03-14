import { User } from '../../types';
import { useState } from 'react';
import Chevron from '../../assets/icons/chevron-light.png';

interface NavBodyProps {
    user: User;
}

function NavBody({ user }: NavBodyProps) {
    const [showMessages, setShowMessages] = useState<boolean>(true);
    const [showCommunities, setShowCommunities] = useState<boolean>(true);

    function toggleMessages() {
        setShowMessages(!showMessages);  
    }

    function toggleCommunities() {
        setShowCommunities(!showCommunities);  
    }

    return (
        <>
            <div className="nav-section" id="nav-communities-container">
                <div className="divider" onClick={toggleCommunities}>
                    <label>Communities</label>
                    <img src={Chevron} alt="communities icon" />
                </div>
                { showCommunities && (
                    <div className="content">
                        {user.communities.map((c) => {
                            return <p>{c.name}</p>
                        })}
                    </div>
                )}
            </div>
            <div className="nav-section" id="nav-messages-container">
                <div className="divider" onClick={toggleMessages}>
                    <label>Messages</label>
                    <img src={Chevron} alt="chatter profile pic" />
                </div>
                { showMessages && (
                    <div className="content">
                        {user.chats.map((c) => {
                            return <p>{c.user.name}</p>
                        })}
                    </div>
                )}
            </div>
        </>
    );
}

export default NavBody;
