import './Dashboard.css';
import SettingsIcon from '../../assets/icons/settings-light.png';
import NavBody from './NavBody';
import { useContext } from 'react';
import { DashBoardContext } from './Dashboard';

function Navbar() {
    const context = useContext(DashBoardContext);
    const user = context?.user;

    return (
        <div className="nav-bar">
            <div className="nav-logo">
                <h1>torg</h1>
            </div>
            <div className="nav-body">
                <NavBody />
            </div>
            <div id="nav-footer" onClick={() => {
                if (context.setActiveTab) {
                    context.setActiveTab({type: 'user', data: user});
                }
            }}>
                <div className='user'>
                    <img src={user.picture} alt="Profile picture" className="profile-icon"/>
                    <p>{user.name}</p>
                </div>
                <img
                    src={SettingsIcon}
                    alt="Settings icon"
                    className="settings-icon"
                />
            </div>
        </div>
    );
};

export default Navbar;
