import { Routes, Route, Link, BrowserRouter as Router } from 'react-router-dom';
import AddMember from './AddMember';
import GetMember from './GetMember';
import SidePanel from './SidePanel';

import './Org.css'
import Header from './Header';

const Org = () => {
    return (
        // <div className="main-div">
        //     <div className="side-panel">
        //         <Link to="/addMember">
        //             <button>Add Member</button>
        //         </Link>
        //         <Link to="/getMember">
        //             <button>Get Member</button>
        //         </Link>
        //         <button>Add Member</button>
        //         <button>Add Member</button>
        //         <button>Add Member</button>
        //         <button>Add Member</button>
        //         <button>Add Member</button>
        //         <button>Add Member</button>
        //         <button>Add Member</button>
        //     </div>
        //     <div className="content-main">
        //     <Routes>
        //         <Route path='/addMember' element={<AddMember />} ></Route>
        //         <Route path='/getMember' element={<GetMember />}></Route>
        //     </Routes>
        //     </div>
        // </div>
        // <></>
        <>
            <Header />
            <SidePanel />
        </>
    );
};

export default Org;