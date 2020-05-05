const { Runtime } = require("near-sdk-as/runtime");
const path = require("path");

const contractPath = path.join(__dirname, "../out/main.wasm");

describe('Token', function () {
  let runtime;
  let contract;

  function getCounter() {
    return contract.view("get_num").return_data;
  }

  beforeAll(function () {
    runtime = new Runtime();
    contract = runtime.newAccount("counter", contractPath);
  });

  describe('counter', function () {
    it('can be incremented', function () {
      const startCounter = getCounter() 
      expect(startCounter).toEqual(0);
      contract.call("increment");
      const endCounter = getCounter()
      expect(endCounter).toEqual(startCounter + 1);
    });

    it('can be decremented', function () {
      const startCounter = getCounter()
      contract.call("decrement");
      const endCounter = getCounter()
      expect(endCounter).toEqual(startCounter - 1);
    });

    it("should be resetable", () => {
      contract.call("increment");
      contract.call("reset");
      expect(getCounter()).toEqual(0);
    })
  });
});