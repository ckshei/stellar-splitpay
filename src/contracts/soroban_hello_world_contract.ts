import * as Client from 'soroban_hello_world_contract';
import { rpcUrl } from './util';

export default new Client.Client({
  ...Client.networks.testnet,
  rpcUrl,
  allowHttp: true,
});
