import { ChangeEvent, useEffect, useState } from 'react';
import { Navigate, Route, BrowserRouter as Router, Routes } from 'react-router-dom';

import '@polkadot/api-augment';

import { ApiPromise } from '@polkadot/api';
import { WsProvider } from '@polkadot/rpc-provider';

import { InjectedAccountWithMeta } from '@polkadot/extension-inject/types';
import { web3Accounts, web3Enable } from '@polkadot/extension-dapp';

import './App.css';
import Org from './Pages/Org';
import Project from './Pages/Project';

function App() {
    const [accounts, setAccounts] = useState<InjectedAccountWithMeta[]>([]);
    const [selectedAcccount, setSelectedAccount] = useState<InjectedAccountWithMeta>();

    const setup = async () => {
        const wsProvider = new WsProvider('wss://shiden.api.onfinality.io/public-ws');
        const api = await ApiPromise.create({ provider: wsProvider });

        //console.log(api.genesisHash.toHex());
        //console.log((await api.rpc.system.properties()).toHuman());
    };

    useEffect(() => {
        setup();
    }, []);

    const handleConnection = async () => {
        const extensions = await web3Enable('management-app');
        if (extensions.length === 0) {
            return;
        }
        const allAccounts = await web3Accounts();
        //console.log(allAccounts);

        setAccounts(allAccounts);
    };

    const handleAccountSelection = (e: ChangeEvent<HTMLSelectElement>) => {
        const selectedAddress = e.target.value;

        const account = accounts.find(account => {
            console.log(account.address);
            return account.address === selectedAddress ; 
        });

        if (account === undefined) {
            throw Error("No Account Found!!");
        }

        setSelectedAccount(account);

        console.log("Selected Account is " , selectedAcccount);

        return <Navigate to="/org" replace={true}/>;
    };

    return (
        <div className='App'>
            <Router>
                <Routes>
                    <Route path='/' element={
                        <header className="App-header">
                            <div className="Wall">Connect To Wallet</div>
                            {accounts.length === 0 ?
                                (<button onClick={handleConnection}> Select Account </button>) : null}

                            {accounts.length > 0 && !selectedAcccount ? (<>
                                <select defaultValue={""} onChange={handleAccountSelection}>
                                    <option value="" disabled selected hidden>Choose your Account</option>
                                    {accounts.map((account) => <option value={account.address}>{account.address}</option>)}
                                </select>
                            </>)
                                : null}
                        </header>}></Route>
                    <Route path='/org' element={<Org />}></Route>
                    <Route path='/project' element={<Project />}></Route>
                </Routes>
            </Router>
        </div>

    );
}

export default App;
