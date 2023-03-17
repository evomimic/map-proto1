import { assert, test } from "vitest";

import { runScenario, pause, CallableCell, } from '@holochain/tryorama';
import { AgentPubKey, Entry, NewEntryAction, ActionHash, EntryHash, Record, AppBundleSource,  fakeActionHash, fakeAgentPubKey, fakeEntryHash } from '@holochain/client';
import { decode } from '@msgpack/msgpack';

//import { Record, Entry, ActionHash, EntryHash, AgentPubKey } from '@holochain/client';
import { Base64 } from 'js-base64';

export function deserializeHash(hash: string): Uint8Array {
  return Base64.toUint8Array(hash.slice(1));
}

export function serializeHash(hash: Uint8Array): string {
  return `u${Base64.fromUint8Array(hash, true)}`;
}
export interface HolonDescriptor { 
  header: TypeHeader
}

export interface TypeHeader {
  type_name: string,
  description: string,
}


test('get all holon types', async () => {
  await runScenario(async scenario => {

       // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + '/../workdir/map-proto1.happ';

    // Set up the app to be installed 
    const appSource = { appBundleSource: { path: testAppPath } };

   // Add 2 players with the test app to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithApps([appSource, appSource]);

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents();
  
   //  TRYORAMA_LOG_LEVEL=debug node test.js
    
   //test for existing holons:
    console.log("Bob gets all holon types:")
    let collectionOutput: Record[] = await bob.cells[0].callZome({
      zome_name: "hc_zome_coordinator_externs",
      fn_name: "get_all_holontypes",
      payload: null
    });
    console.log(collectionOutput)
    assert.isAtLeast(collectionOutput.length, 1);
  
  });
});