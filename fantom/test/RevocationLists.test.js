const { expect } = require("chai");
const { ethers } = require("hardhat");



describe("RL2020", function () {
  it("Should create a empty list", async function () {
    const RL2020 = await ethers.getContractFactory("RevocationLists");
    const rl2020 = await RL2020.deploy();
    await rl2020.deployed();

    await rl2020.register_list("my.list");
    expect(await rl2020.get_encoded_list("my.list")).to.equal("0".repeat(4096 * 2))
  }),
    it("Should set a bit index to 1", async function () {
      const RL2020 = await ethers.getContractFactory("RevocationLists");
      const rl2020 = await RL2020.deploy();
      await rl2020.deployed();

      const id = "my.list.1";

      await rl2020.register_list(id);
      expect(await rl2020.get_encoded_list(id)).to.equal("0".repeat(4096 * 2))

      expect(await rl2020.is_set(id, 1)).to.equal(false);
      await rl2020.set(id, [1], []);
      expect(await rl2020.is_set(id, 1)).to.equal(true);
    }),
    it("Should set a bit index to 0", async function () {
      const RL2020 = await ethers.getContractFactory("RevocationLists");
      const rl2020 = await RL2020.deploy();
      await rl2020.deployed();

      const id = "my.list.2";

      await rl2020.register_list(id);
      expect(await rl2020.get_encoded_list(id)).to.equal("0".repeat(4096 * 2))

      expect(await rl2020.is_set(id, 1)).to.equal(false);
      await rl2020.set(id, [1], []);
      expect(await rl2020.is_set(id, 1)).to.equal(true);
      await rl2020.set(id, [], [1]);
      expect(await rl2020.is_set(id, 1)).to.equal(false);
    })
});