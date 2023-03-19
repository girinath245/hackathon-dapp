import Card from './Card'
import Header from './Header'
import SidePanel from './SidePanel'
import './ProjectList.css';
//import { Abi, ContractPromise } from '@polkadot/api-contract'
//import { ApiPromise , WsProvider} from '@polkadot/api';
//import ORGABI from '../../contract-json/org.json' 

function ProjectList() {
  /*
  let api : ApiPromise, orgContract : ContractPromise , abi : Abi ;
  const address = 'XZHnvh88h7mS3fJSBV19EfUy3ZEU4qtQPC9gQ8WTkGcPQmi' ;
  */

  let headers = [] ;

  /*
  const setupContract = async () => {
    /*
    const wsProvider = new WsProvider('wss://shiden.api.onfinality.io/public-ws');
    api = await ApiPromise.create({ provider: wsProvider });

    await api.isReady ;
    
    console.log(api.genesisHash.toHex());

    abi = new Abi(ORGABI)

    orgContract = new ContractPromise(api, abi, address)
    console.log((await api.rpc.system.properties()).toHuman());
    
    const total = await orgContract?.query.total_projects(); 
    */
   
  //};
  
  const total = 3;
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