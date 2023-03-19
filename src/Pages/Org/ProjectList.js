import React, { useContext } from 'react'
import Card from './Card'
import Header from './Header'
import SidePanel from './SidePanel'
import './ProjectList.css';
import { Abi, ContractPromise } from '@polkadot/api-contract'
import { ApiPromise , WsProvider} from '@polkadot/api';
import ORGABI from '../../../contract/org/target/ink/org.json' 

async function ProjectList() {
  let api , orgContract , abi;
  const address = '12343' ;

  const setup = async () => {
    const wsProvider = new WsProvider('wss://shiden.api.onfinality.io/public-ws');
    api = await ApiPromise.create({ provider: wsProvider });

    await api.isReady ;
    
    console.log(api.genesisHash.toHex());

    abi = new Abi(ORGABI)

    orgContract = new ContractPromise(api, abi, address)
    console.log((await api.rpc.system.properties()).toHuman());
  };

  await setup();

  const headers = [];

  const total = await orgContract?.query.total_projects(); 

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