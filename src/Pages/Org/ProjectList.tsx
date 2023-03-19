import React from 'react'
import Card from './Card'
import Header from './Header'
import SidePanel from './SidePanel'

import './ProjectList.css';

function ProjectList() {
  const headers = [];

  const total = 4;

  for (let index = 0; index < total; index++) {
    headers.push(<Card key={index} />);
  }

  return (
    <>
        <Header />
        <SidePanel />
        <div className='card-collection'>
            {headers}
        </div>
    </>
  )
}

export default ProjectList