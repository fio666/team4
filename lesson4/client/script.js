import { ApiPromise, WsProvider, Keyring } from '@polkadot/api';
import { blake2AsHex } from '@polkadot/util-crypto';
import fs from "fs";

const convert = (from, to) => str => Buffer.from(str, from).toString(to)
const utf8ToHex = convert('utf8', 'hex')
const hexToUtf8 = convert('hex', 'utf8')

// config
const WEB_SOCKET = 'ws://localhost:9944';

async function connect() {
  // Construct
  const wsProvider = new WsProvider(WEB_SOCKET);
  const api = await ApiPromise.create({
    provider: wsProvider,
    types: {
      'Address': 'AccountId',
      'LookupSource': 'AccountId',
      'Price': 'u128',
    }
  });
  // Retrieve the chain & node information information via rpc calls
  const [chain, nodeName, nodeVersion] = await Promise.all([
    api.rpc.system.chain(),
    api.rpc.system.name(),
    api.rpc.system.version()
  ]);
  console.log(`You are connected to chain ${chain} using ${nodeName} v${nodeVersion}`);
  return api;
}

async function submitDocInfo(filePath, comment) {
  console.debug(`submitDocInfo: ${filePath}, ${comment}`);
  try {
    const api = await connect();

    /******
     * 学员们在这里追加逻辑
     *
     * 把 filePath 档档案通过 hash 函数算出它的 hash 值。然后和 comment 一起提交个 extrinsics
     *   到 Substrate。
     ******/
     let content = fs.readFileSync(filePath, 'utf-8');
     let hasher = blake2AsHex(content);
     const keyring = new Keyring({ type: 'sr25519' });
     const alice = keyring.addFromUri('//Bob', {name: 'Bob'});
     const hash = await api.tx.poeModule.createClaim(hasher, comment).signAndSend(alice);
     console.log('done with hash: ', hash.toString());


  } catch (err) {
    console.error(`Connect to Substrate error:`, err);
    process.exit(1);
  }

  process.exit(0);
}

async function getUserDocs(acct) {
  console.debug(`getUserDocs: ${acct}`);
  try {
    const api = await connect();

    /******
     * 学员们在这里追加逻辑
     *
     * 通过用户 addr, 取得他所有的创建文件的 hash及相关资料。返回值是：
     * {
     *   "0xabcd1234...": ["my note1", 3],
     *   "0xabcd1235...": ["my note2", 5],
     *   "0xabcd1236...": ["my note3", 7],
     *   ...
     * }
     ******/

     const claims = await api.query.poeModule.accountProofs(acct);
     for(const c of claims) {
       let info = await api.query.poeModule.proofs(c);
       info = info.toJSON();
       const memo = hexToUtf8(info[3].slice(2));
       const blockNumber = info[1];
       console.log(`"${c}": ["${memo}", ${blockNumber}]`)
     }

  } catch (err) {
    console.error(`Connect to Substrate error:`, err);
  }

  process.exit(0);
}

function main() {
  const args = process.argv.slice(2, 5);
  switch (args[0]) {
    case 'submitDocInfo':
      submitDocInfo(args[1], args[2])
      break;
    case 'getUserDocs':
      getUserDocs(args[1]);
      break;
    default:
      console.error('Unknown subcommand. Please use `submitDocInfo` or `getUserDocs` only.')
  }
}

main();