
It has been a difficult year for blockchain projects, there have been huge scandals and the trust in the technology and the people have never been so low. But hey, it can still get worse! 

Reflecting on recent developments, at least in the consumer market, in the current decade there have been IMHO, Tesla, it gave a big nudge to the electric car market, OpenAI that with DALL-E and ChatGPT is taking the AI to a greater public, Boston Dynamics that is bringing robotics to the next level and IoT that are slowly but surely entering consumer life via smart home devices. 

Is it the future we were imagining? Not yet, but we are getting there. 

And what about blockchain? That is a controversial topic to say the least: there has been a steady growth in interest about the technology, bot in a positive (google, amazon, finance institution testing it), and in a bad way (hetzner), and there has been some major fuck up to say the least. And the reason is not a secret, the principal use of the technology, as it started with Bitcoin, has been financial: and that's a topic that rarely is covered with positive vibes, add to that the possibility to access the unregulated crypto assets market by anyone combined with the promise of easy money, and what can go wrong?

But I still think there is value in the tech (beside being very interesting to work with as a tech person), and specifically there are a few topics that I find very interesting, beside the financial parts, like, for example governance and support for self-managed identities (SMI or SSI). 

On the identities, and specifically around credentials, there is a specific problem that has a very important role in the credential system, that is, revocation lists. Revocation lists are buckets of credentials identifiers that must be public and tamper proof, that is, only the credential issuer shall be able to update a revocation lists, but every credential verifier (NOTE add link) must be able to query the revocation list. 

Since the revocation lists are public, there is naturally a privacy problem: a third party shall not have the ability to guess if my credential is listed in a revocation list. A few approaches have been developed to address this specific problem, some of them rely on ZK, some of them are proprietary. But there is one of them that provides a good level of privacy and that it is very easily developed using smart contract, and that is going to be the exercise of today: a smart contract that implements in a functional revocation list.

To make the exercise more interesting, is going to be not 1, but 3 implementations, all of them in Rust, targeting 3 different blockchain and smart contract implementations:
- NEAR
- Archway (Cosmowasm)
- Solana



