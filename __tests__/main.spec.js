const { Runtime } = require("near-sdk-as/runtime");
const path = require("path");

const counterPath  = path.join(__dirname, "../out/rust_counter_tutorial.wasm");
const donationPath = path.join(__dirname, "../out/rust_donation_tutorial.wasm");

describe('Token', function () {
  let runtime;
  let counter;

  function getNum(name = "donation") {
    return counter.view("get_num", {account: name }).return_data;
  }

  beforeAll(function () {
    runtime = new Runtime();
    counter = runtime.newAccount("counter", counterPath);
    donation = runtime.newAccount("donation", donationPath);
  });

  describe('donation', function () {
    it('can increment counter', function () {
      expect(getNum()).toBe(0);
      donation.call("increment_my_number", {account_id: "counter"});
      expect(getNum()).toBe(1);
    });
  });
});