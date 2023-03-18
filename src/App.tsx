import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';

import '@polkadot/api-augment';

import Org from './Pages/Org';
import Login from './Pages/Login';
import AddMember from './Pages/AddMember';
import GetMember from './Pages/GetMember';
import Projects from './Pages/Projects';
import Secretory from './Pages/Secretory';
import Proposal from './Pages/Proposal';

function App() {
    return (
        <div className='App'>
            <Router>
                <Routes>
                    <Route path='/' element={<Login />} ></Route>
                    <Route path='/org' element={<Org />}></Route>
                    <Route path='/org/projects' element={<Projects />}></Route>
                    <Route path='/org/secretory' element={<Secretory />}></Route>
                    <Route path='/org/proposal' element={<Proposal />}></Route>
            </Routes>
        </Router>
        </div >

    );
}

export default App;
