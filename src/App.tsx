import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';

import '@polkadot/api-augment';

import Org from './Pages/Org';
import Login from './Pages/Login';
import Project from './Pages/Project';

import ProjectList from './Pages/Org/ProjectList';
import Secretory from './Pages/Org/Secretory';
import Proposal from './Pages/Org/Proposal';

import ProjectInfo from './Pages/Project/ProjectInfo';
import ProjectTasks from './Pages/Project/ProjectTasks';
import ProjectMembers from './Pages/Project/ProjectMembers';

function App() {
    return (
        <div className='App'>
            <Router>
                <Routes>
                    <Route path='/' element={<Login />} ></Route>

                    <Route path='/org' element={<Org />}></Route>
                    <Route path='/org/projectlist' element={<ProjectList />}></Route>
                    <Route path='/org/secretory' element={<Secretory />}></Route>
                    <Route path='/org/proposal' element={<Proposal />}></Route>

                    <Route path='/project' element={<Project />}></Route>
                    <Route path='/project/info' element={<ProjectInfo />}></Route>
                    <Route path='/project/members' element={<ProjectMembers />}></Route>
                    <Route path='/project/tasks' element={<ProjectTasks />}></Route>

            </Routes>
        </Router>
        </div >

    );
}

export default App;
