import './Dashboard.css';
import { useContext, useState } from 'react';
import Chevron from '../../assets/icons/chevron-light.png';
import { DashBoardContext } from './Dashboard';

function NavBody() {
    const context = useContext(DashBoardContext);
    const user = context.user;
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
                        {user.communities.map((c, i) => {
                            return <p key={i} className='community-tab' onClick={() => {
                                if (context.setActiveTab) {
                                    context.setActiveTab({type: 'community', data: c});
                                }
                            }} >{c.name}</p>
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
                        {user.chats.map((c, i) => {
                            return <p key={i} className='community-tab' onClick={() => {
                                if (context.setActiveTab) {
                                    context.setActiveTab({type: 'chat', data: c});
                                }
                            }}>{c.name}</p>
                        })}
                    </div>
                )}
            </div>
        </>
    );
}

export default NavBody;
