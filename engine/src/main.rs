/*
Order Matching Flow
-------------------
BID = Buy price (what the buyer is willing to pay)
ASK = Sell price (what the seller is willing to accept)

1. Insert the new order into the appropriate side of the orderbook
   - If it's a BUY order → compare with the lowest ASK
   - If it's a SELL order → compare with the highest BID

2. Matching Logic:
   Case 1: BUY order comes in
       if buy_price >= lowest_sell_price (ASK)
           match the order at the sell price
           transfer quantity from seller to buyer
           transfer funds from buyer to seller
           reduce quantity of both orders
           if either order is fully filled → remove it from the orderbook
           continue matching until the BUY order is filled or no matching ASK exists
       else 
           insert the BUY order into the orderbook

   Case 2: SELL order comes in
       if sell_price <= highest_buy_price (BID)
           match the order at the buy price
           transfer quantity from seller to buyer
           transfer funds from buyer to seller
           reduce quantity of both orders
           if either order is fully filled → remove it from the orderbook
           continue matching until the SELL order is filled or no matching BID exists
       else
           insert the SELL order into the orderbook

3. If there is still unmatched quantity after matching:
       - Add the remaining order to the orderbook at its price level
       - Maintain sorted order (highest BID first, lowest ASK first)
       - Maintain FIFO for orders with the same price

4. Broadcast orderbook updates to all market participants
       - WebSocket, Kafka, etc.

Notes:
- Market orders skip step 3 (they just keep matching until filled or book is empty)
- CEX systems usually process these in-memory with a single-threaded event loop per market
- Each trading pair (BTC/USDT, ETH/USDT, etc.) often has its own matching engine instance
*/
