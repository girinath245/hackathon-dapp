import React from 'react'
import Header from './Header'
import ListRow from './ListRow'
import SidePanel from './SidePanel'

import './Secretory.css'

function Secretory() {
  return (
    <>
      <Header />
      <SidePanel />

      <div className='member-container'>
        <div className='member-section'>
          <h2>Secretory</h2>
            <ListRow />
            <ListRow />
            <ListRow />
            <ListRow />
            <ListRow />
        </div>
        <div className='member-section'>
          <h2>Member</h2>
            <ListRow />
            <ListRow />
            <ListRow />
            <ListRow />
            <ListRow />
        </div>
      </div>
    </>
  )
}

export default Secretory