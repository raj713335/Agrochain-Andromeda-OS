import React, { useContext, useState } from 'react';
import { NftContext } from '../frontend/NftContext/NftProvider';

export default function AOSMintList() {
    const { aosClient, carbonAdoAddress } = useContext(NftContext);
    const [mint, setMint] = useState({ amount: '', iot_device_id: '', location: '', practice: 'organic' });
    const [list, setList] = useState({ credit_id: '', price: '', quantity: '' });
    const [busy, setBusy] = useState(false);

    const mintCredit = async () => {
        if (!aosClient) return;
        setBusy(true);
        try {
            const msg = {
                mint_carbon_credit: {
                    carbon_amount: mint.amount,
                    verification_data: {
                        iot_device_id: mint.iot_device_id,
                        sensor_readings: [],
                        location: mint.location,
                        farming_practices: [mint.practice],
                        verification_timestamp: { seconds: Math.floor(Date.now()/1000), nanos: 0 },
                        verification_score: '0.80'
                    },
                    farming_practices: [mint.practice]
                }
            };
            await aosClient.client.execute(aosClient.signerAddress, carbonAdoAddress, msg, 'auto');
            alert('Minted');
        } catch (e) {
            console.error(e);
            alert('Failed');
        } finally { setBusy(false); }
    };

    const listCredit = async () => {
        if (!aosClient) return;
        setBusy(true);
        try {
            const msg = { list_carbon_credit: { credit_id: list.credit_id, price: list.price, quantity: list.quantity } };
            await aosClient.client.execute(aosClient.signerAddress, carbonAdoAddress, msg, 'auto');
            alert('Listed');
        } catch (e) { console.error(e); alert('Failed'); } finally { setBusy(false); }
    };

    return (
        <div className="container mt-4">
            <h2>aOS Mint & List</h2>
            <div className="card p-3 mb-3">
                <h5>Mint</h5>
                <div className="row g-2">
                    <div className="col-md-3"><input className="form-control" placeholder="Amount" onChange={(e)=>setMint({...mint, amount: e.target.value})} /></div>
                    <div className="col-md-3"><input className="form-control" placeholder="IoT Device Id" onChange={(e)=>setMint({...mint, iot_device_id: e.target.value})} /></div>
                    <div className="col-md-3"><input className="form-control" placeholder="Location" onChange={(e)=>setMint({...mint, location: e.target.value})} /></div>
                    <div className="col-md-3"><input className="form-control" placeholder="Practice" onChange={(e)=>setMint({...mint, practice: e.target.value})} defaultValue={'organic'} /></div>
                </div>
                <button disabled={busy || !aosClient} className="btn btn-primary mt-3" onClick={mintCredit}>Mint</button>
            </div>
            <div className="card p-3">
                <h5>List</h5>
                <div className="row g-2">
                    <div className="col-md-4"><input className="form-control" placeholder="Credit Id" onChange={(e)=>setList({...list, credit_id: e.target.value})} /></div>
                    <div className="col-md-4"><input className="form-control" placeholder="Price (uandr)" onChange={(e)=>setList({...list, price: e.target.value})} /></div>
                    <div className="col-md-4"><input className="form-control" placeholder="Quantity" onChange={(e)=>setList({...list, quantity: e.target.value})} /></div>
                </div>
                <button disabled={busy || !aosClient} className="btn btn-secondary mt-3" onClick={listCredit}>List</button>
            </div>
        </div>
    );
}


