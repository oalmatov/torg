import './Dashboard.css';
import { User } from '../../types';
import SettingsIcon from '../../assets/icons/settings-light.png';
import NavBody from './NavBody';

interface NavbarProps {
    user: User;
}

function Navbar({ user }: NavbarProps) {
    return (
        <div className="nav-bar">
            <div className="nav-logo">
                <h1>torg</h1>
            </div>
            <div className="nav-body">
                <NavBody user={user} />
            </div>
            <div id="nav-footer">
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
