import { Injectable, OnDestroy } from "@angular/core";
import { environment } from 'src/environments/environment';
import { AppSignalCb, AppSignal, AppWebsocket, CellId, InstalledCell, AppInfo, CellInfo, RoleName, CellType, CellProvisioningStrategy, ProvisionedCell } from '@holochain/client'
import { serializeHash } from "../helpers/utils";


export enum ConnectionState{
  OPEN,
  CLOSED,
  CLOSING,
  CONNECTING
}

export type SignalCallback = {cell_name:string, zome_name:string, cb_fn:AppSignalCb }

//tsconfig: "allowSyntheticDefaultImports": true,
@Injectable({
  providedIn: "root"
})
export class HolochainService implements OnDestroy{
  protected appWS!: AppWebsocket
  protected appInfo!: AppInfo 
  protected _cellData!: Record<RoleName, Array<CellInfo>>;
  protected signalCallbacks: SignalCallback[] = []

  get_pub_key_from_cell(cell:string):string | undefined {
    return undefined //this.appInfo.agent_pub_key uni8array
    //for(let installedcell of this._cellData['myrole']){
    //  if (installedcell.role_id == cell)
     //   return serializeHash(installedcell.cell_id[1])
   // };
   // return undefined
  }

  get_installed_cells(){
    return this._cellData
  }

  getCellNameFromDNAHash(dnahash:Uint8Array){
    for(let cell of this.appInfo.cell_info["map-proto1"]){
      if (Object.values(cell)[0].cell_id[0]  == dnahash)
        return Object.values(cell)[0].name
    };
    return undefined
  }

  protected getCellId(cell_name:string):CellId | undefined {
    for(let installedcell of this._cellData["map-proto1"]){
      if (Object.values(installedcell)[0].name == cell_name)
        return Object.values(installedcell)[0].cell_id
    };
    return undefined
  }

    //if this doesnt return a resolved promise.. the app will not bootstrap  
    async init():Promise<void>{ //called by the appModule at startup
        sessionStorage.clear()
          try{
            console.log("Connecting to holochain")
            this.appWS =  await AppWebsocket.connect(environment.HOST_URL,1500)
            this.appWS.on("signal",(s)=>this.signalHandler(s))
            this.appInfo = await this.appWS.appInfo({ installed_app_id: environment.APP_ID});
            this._cellData = this.appInfo.cell_info;
            console.log("Connected to holochain",this.appInfo.cell_info)
            console.log("app status",this.appInfo.status)
            sessionStorage.setItem("status","HOLOCHAIN")
          }catch(error){
              console.error(error)
              if (environment.mock){
                sessionStorage.setItem("status","mock")
                //this._cellData = new Record<"mock",[]>
                  //{cell_id:[new Uint8Array(10), new Uint8Array(10)], role_id:"typedescriptor:role0"},
                  //{cell_id:[new Uint8Array(8), new Uint8Array(8)], role_id:"holon:role0"},
                  //{cell_id:[new Uint8Array(6), new Uint8Array(6)], role_id:"holon:role1"}] 
                return Promise.resolve()
              }
        }
    }

     call(cell:string, zome:string, fn_Name:string, payload:any, timeout=15000): Promise<any>{
       const cellId = this.getCellId(cell)
       if (!cellId) throw new Error("cell not found:"+cell);
        return this.appWS.callZome(
          {
            cap_secret: null,
            cell_id: cellId,
            zome_name: zome,
            fn_name: fn_Name,  //will always be execute
            payload: payload,  // specify actually commmand function call
            provenance: cellId[1],
          },
        timeout
        );
      }

    /* in the future 'zome_name' and 'cell_name' should be meta-data of AppSignal and Not part of the payload*/
    signalHandler(signal: AppSignal): void {
      if(this.signalCallbacks.length > 0){
        for (const cb of this.signalCallbacks) {
          console.log("cb data: ",cb)
          var signalCellName = this.getCellNameFromDNAHash(signal.cell_id[0])
          if (cb.cell_name == signalCellName && cb.zome_name == signal.zome_name){
            console.log("signal callback found, executing cb function: ")
            cb.cb_fn(signal)
            return
          }
        }
        console.log("Signal handler for signal was not found",signal)
      }
    }

    registerCallback(cell_name:string, zomes: string[], handler:AppSignalCb){
      zomes.forEach(zome => {
        this.signalCallbacks.push({cell_name:cell_name, zome_name:zome, cb_fn:handler})
      });
    }

    //TODO add event listener and relay state change back to UI
    getConnectionState():string{
      if (this.appWS)
        return ConnectionState[this.appWS.client.socket.readyState]
      else
        return ConnectionState[1]
    }

    ngOnDestroy(){
      this.appWS.client.close();
    }

}
