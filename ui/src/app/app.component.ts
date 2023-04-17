import { Component, Provider } from '@angular/core';
import { HolonReceptor } from './receptors/holon.receptor';
import { HolochainService } from './services/holochain.service';
//import { HolonStoreProvider } from './services/store.factory.service';
import { StoreFactory, TypeDescriptorStoreProvider } from './services/store.factory.service';
import { TypeDescriptorStore } from './stores/typedescriptor.store';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  providers:[TypeDescriptorStoreProvider]
})
export class AppComponent {
  title = 'map-ts';
  status:string | null = ""
  statusStyling:string = "text-green-500"

  constructor(private _store:TypeDescriptorStore, private hcs:HolochainService){//readonly _stores:StoreFactory) {
     console.log(this._store.id) //to check we are using the same instance in the component tree
  }

  ngOnInit(): void {
    const mode = " (Mode: "+sessionStorage.getItem("status")+")"
    const stat = this.hcs.getConnectionState()
    this.status = mode+" "+stat
    switch (stat) {
      case "CLOSED":
        this.status = "Socket status: "+stat+mode
        this.statusStyling = "text-red-500"
        break;
      case "CLOSING" || "CONNECTING":
        this.status = "Socket status: "+stat+mode
        this.statusStyling = "text-yellow-500"
        break;
      case "OPEN":
        this.status = "Socket status: "+stat+mode
        this.statusStyling = "text-green-500"
        break;
      default:
        break;
    }
    
    this._store.getAllTypeDescriptors() //network pull to initialize store state.. 
  }
}
