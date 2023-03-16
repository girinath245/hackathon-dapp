import { useNavigate } from 'react-router-dom';

import { InjectedAccountWithMeta } from '@polkadot/extension-inject/types';
import { web3Accounts, web3Enable } from '@polkadot/extension-dapp';

import { ChangeEvent, useEffect, useState } from 'react';

import '@polkadot/api-augment';

import { ApiPromise } from '@polkadot/api';
import { WsProvider } from '@polkadot/rpc-provider';

const Login = () => {
    const [accounts, setAccounts] = useState<InjectedAccountWithMeta[]>([]);
    const [selectedAcccount, setSelectedAccount] = useState<InjectedAccountWithMeta>();
    const navigate = useNavigate();

    const setup = async () => {
        const wsProvider = new WsProvider('wss://shiden.api.onfinality.io/public-ws');
        const api = await ApiPromise.create({ provider: wsProvider });

        console.log(api.genesisHash.toHex());
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

        if (allAccounts.length === 1) {
            setSelectedAccount(allAccounts[0]);
        }
    };

    const handleAccountSelection = (e: ChangeEvent<HTMLSelectElement>) => {
        const selectedAddress = e.target.value;

        console.log(selectedAddress)
        const account = accounts.find(account => account.address === selectedAddress);

        setSelectedAccount(account);

        navigate("/org");
    };

    return (
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
    </header>
    );
};

export default Login;
