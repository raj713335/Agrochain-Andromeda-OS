import React, { useEffect, useState, useContext } from 'react';
import { NftContext } from '../frontend/NftContext/NftProvider';

// Minimal CosmJS client lazy import to avoid bundling issues if Keplr is not present
const useAosClient = () => {
    const [cosmwasm, setCosmwasm] = useState(null);
    useEffect(() => {
        (async () => {
            try {
                const m = await import('@cosmjs/cosmwasm-stargate');
                setCosmwasm(m);
            } catch (e) {
                // ignore
            }
        })();
    }, []);
    return cosmwasm;
};

export default function CarbonMarketplace() {
    const { aosClient, carbonAdoAddress } = useContext(NftContext);
    const [listings, setListings] = useState([]);
    const [loading, setLoading] = useState(false);

    const loadListings = async () => {
        if (!aosClient || !carbonAdoAddress) return;
        setLoading(true);
        try {
            const res = await aosClient.queryContractSmart(carbonAdoAddress, { list_listings: { start_after: null, limit: 50 } });
            const items = res.listings || res || [];
            setListings(items);
        } catch (e) {
            console.error('Failed to query listings', e);
        } finally {
            setLoading(false);
        }
    };

    const handleBuy = async (listing) => {
        if (!aosClient || !carbonAdoAddress) return;
        try {
            const fee = '200000';
            await aosClient.signAndBroadcast(
                aosClient.signerAddress,
                [
                    {
                        typeUrl: '/cosmwasm.wasm.v1.MsgExecuteContract',
                        value: {
                            sender: aosClient.signerAddress,
                            contract: carbonAdoAddress,
                            msg: new TextEncoder().encode(JSON.stringify({ buy_carbon_credit: { listing_id: listing.id } })),
                            funds: [{ denom: 'uandr', amount: (Number(listing.price) * Number(listing.quantity)).toString() }]
                        }
                    }
                ],
                { amount: [{ denom: 'uandr', amount: '2500' }], gas: fee }
            );
            await loadListings();
        } catch (e) {
            console.error('Buy failed', e);
        }
    };

    useEffect(() => {
        loadListings();
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [aosClient, carbonAdoAddress]);

    if (!aosClient) return <div className="container mt-4">Connect Keplr to view aOS marketplace.</div>;
    if (loading) return <div className="container mt-4">Loading carbon listings...</div>;

    return (
        <div className="container mt-4">
            <h2>Carbon Marketplace (aOS)</h2>
            <div className="row mt-3">
                {listings.length === 0 && <div className="col-12">No active listings</div>}
                {listings.map((l, idx) => (
                    <div className="col-md-4" key={idx}>
                        <div className="card mb-3">
                            <div className="card-body">
                                <h5 className="card-title">{l.carbon_credit_id}</h5>
                                <p className="card-text">Price: {l.price} uandr</p>
                                <p className="card-text">Quantity: {l.quantity}</p>
                                <button className="btn btn-primary" onClick={() => handleBuy(l)}>Buy</button>
                            </div>
                        </div>
                    </div>
                ))}
            </div>
        </div>
    );
}


