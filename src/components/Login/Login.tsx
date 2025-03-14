import { ReactElement, useState } from 'react';
import './Login.css';

function LoginPage(): ReactElement {
    const [formType, setFormType] = useState<'login' | 'signup' | 'forgot'>('login');

    const handleToggleForm = (form: 'login' | 'signup' | 'forgot') => {
        setFormType(form);
    };

    return (
        <section className="login-container">
            <div className="torg-container">
                <h1>torg</h1>
            </div>
            <div className="auth-container">
                {formType === 'login' && (
                    <form id="login-form" className="auth-form">
                        <h1>Login</h1>
                        <label>Email</label>
                        <input type="email" placeholder="torgin.terry@example.com" required />
                        <div className="pw-label">
                            <label>Password</label>
                            <a className="toggle-forgot" onClick={() => handleToggleForm('forgot')}>
                                Forgot your password?
                            </a>
                        </div>
                        <input type="password" required />
                        <button type="submit">Log in</button>
                        <div className="sign-up-link">
                            <p>
                                Don't have an account?{' '}
                                <a className="toggle-signup" onClick={() => handleToggleForm('signup')}>
                                    Sign up!
                                </a>
                            </p>
                        </div>
                    </form>
                )}

                {formType === 'signup' && (
                    <form id="signup-form" className="auth-form">
                        <h1>Sign up</h1>
                        <label>Email</label>
                        <input type="email" placeholder="torgin.terry@example.com" required />
                        <label>Password</label>
                        <input type="password" required />
                        <label>Confirm password</label>
                        <input type="password" required />
                        <button type="submit">Sign up</button>
                        <div className="sign-up-link">
                            <p>
                                Have an account?{' '}
                                <a className="toggle-login" onClick={() => handleToggleForm('login')}>
                                    Log in!
                                </a>
                            </p>
                        </div>
                    </form>
                )}

                {formType === 'forgot' && (
                    <form id="forgot-form" className="auth-form">
                        <h1>Restore password</h1>
                        <label>Email</label>
                        <input type="email" placeholder="torgin.terry@example.com" required />
                        <label>New password</label>
                        <input type="password" required />
                        <label>Confirm new password</label>
                        <input type="password" required />
                        <button type="submit">Restore password</button>
                        <div className="sign-up-link">
                            <p>
                                Remembered your password?{' '}
                                <a className="toggle-login" onClick={() => handleToggleForm('login')}>
                                    Log in!
                                </a>
                            </p>
                        </div>
                    </form>
                )}
            </div>
        </section>
    );
};

export default LoginPage;
