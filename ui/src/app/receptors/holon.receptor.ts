//import { AgentPubKeyB64, EntryHashB64 } from "../helpers/utils";
import { AgentPubKey, AppSignal, Record } from '@holochain/client';
import { HolochainService } from '../services/holochain.service';
import { Subject } from 'rxjs';
import { environment } from 'src/environments/environment';
import { HolonEntry, mockHolonEntryArray } from '../models/holon'
import { Injectable } from '@angular/core';

//injected in the store
@Injectable()
export class HolonReceptor {
  private _cellname!:string
  private _zomes:string[] = ["holon"]
  public signalReceived$ = new Subject<HolonEntry>()  //new

 
  constructor(private hcs:HolochainService){//, private _cellname:string) {
   // this.hcs.registerCallback(_cellname,this._zomeName, (s)=>this.signalHandler(s))
  }

  registerCallback(cellname:string){
    this._cellname = cellname
    this.hcs.registerCallback(cellname,this._zomes, (s)=>this.signalHandler(s))
  }

  getAllHolons(): Promise<HolonEntry[]> {
    if (environment.mock || sessionStorage.getItem("status") == "mock")
      return new Promise<HolonEntry[]>((resolve) => {setTimeout(()=> resolve(mockHolonEntryArray),2000)})
    else
      return this.callCell('get_all_holons', 'holon', null);
  }

  getNetworkStatus():string {
    return this.hcs.getConnectionState()
  }

  private callCell(fn_name: string, zome_name:string, payload: any): Promise<any> {
    return this.hcs.call(this._cellname, zome_name, fn_name, payload);
  }

  //future.. make dynamic hashmap lookup
  private signalHandler(sig: AppSignal) {
    //console.log("handler called",payload)
    switch (sig.payload) {
      case 'newholon':
        //this.signalReceived$.next(payload.data.SignalReceived);
        break;

      default:
        break;
    }
  }
}