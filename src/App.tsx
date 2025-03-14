import { BrowserRouter as Router, Route, Routes } from 'react-router-dom';
import Login from './components/Login/Login'
import Dashboard from './components/Dashboard/Dashboard';

function App() {
    return (
        <>
            <Router>
                <div>
                    {/* Define routes here */}
                    <Routes>
                        <Route path="/" element={<Login />} />
                        <Route path="/dashboard" element={<Dashboard />} /> 
                    </Routes>
                </div>
            </Router>
        </>
    )
}

export default App
