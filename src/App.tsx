import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';

import '@polkadot/api-augment';

import './App.css';
import Org from './Pages/Org';
import Project from './Pages/Project';
import Login from './Pages/Login';

function App() {
    return (
        <div className='App'>
            <Router>
                <Routes>
                    <Route path='/' element={<Login />} ></Route>
                    <Route path='/org' element={<Org />}></Route>
                    <Route path='/project' element={<Project />}></Route>
            </Routes>
        </Router>
        </div >

    );
}

export default App;
