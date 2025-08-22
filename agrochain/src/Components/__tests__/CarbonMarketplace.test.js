import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import CarbonMarketplace from '../../Components/CarbonMarketplace';
import { NftContext } from '../../frontend/NftContext/NftProvider';

function renderWithCtx(ctx) {
  return render(
    <NftContext.Provider value={ctx}>
      <CarbonMarketplace />
    </NftContext.Provider>
  );
}

test('renders connect prompt when no aOS client', () => {
  renderWithCtx({ aosClient: null, carbonAdoAddress: '' });
  expect(screen.getByText(/Connect Keplr to view aOS marketplace/i)).toBeInTheDocument();
});

test('lists items from ADO query', async () => {
  const listings = [{ id: 'listing_1', carbon_credit_id: 'credit_1', price: '10', quantity: '2' }];
  const ctx = {
    carbonAdoAddress: 'andr1xxx',
    aosClient: {
      signerAddress: 'andr1abc',
      queryContractSmart: jest.fn().mockResolvedValue({ listings })
    }
  };
  renderWithCtx(ctx);
  expect(await screen.findByText(/credit_1/i)).toBeInTheDocument();
});


