import './Org.css'
import Header from './Org/Header';
import SidePanel from './Org/SidePanel';

import { useLocation } from 'react-router-dom';
  
const Org = () => {
    const location = useLocation();
    console.log("User is ",location.state.accountaddress);
    
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