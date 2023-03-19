import React from 'react'
import { Link } from 'react-router-dom';
import { SideBarData } from './SideBarData';

import './SidePanel.css';

function SidePanel() {
  return (
    <>
        <nav className='nav-menu'>
            <ul className='nav-menu-items'>
                {SideBarData.map((item, index) => {
                    return (
                        <li key={index} className='nav-text'>
                            <Link to={item.path}>
                                <span>{item.title}</span>
                            </Link>
                        </li>
                    )
                })}
            </ul>
        </nav>
    </>
  )
}

export default SidePanel;