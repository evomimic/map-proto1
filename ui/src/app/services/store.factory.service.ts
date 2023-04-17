import { ComponentStore } from '@ngrx/component-store';
import { HolochainService } from './holochain.service';
import { Dictionary, serializeHash } from '../helpers/utils';
import { HolonStore } from '../stores/holon.store';
import { HolonReceptor } from '../receptors/holon.receptor';
import { inject, Inject, Injectable, InjectionToken, Injector, Provider } from '@angular/core';
import { TypeDescriptorStore } from '../stores/typedescriptor.store';
import { TypeDescriptorReceptor } from '../receptors/typedescriptor.receptor';


function holonStoreFactory(hcs: HolochainService):HolonStore[]{
  const cells = hcs.get_installed_cells()
  let storeArray:HolonStore[]= []
  for (const cell of cells['map-proto1']) {
    let cell_name = Object.values(cell)[0].name
    if (cell_name.startsWith("holon:")){
      let holonReceptor = new HolonReceptor(hcs)
      holonReceptor.registerCallback(cell_name)
      storeArray.push(new HolonStore(holonReceptor))
    }
  }
  return storeArray
};

function typeDescriptorStoreFactory(hcs: HolochainService){
  const cells = hcs.get_installed_cells()
  const receptor = new TypeDescriptorReceptor(hcs)
  for (const cell of cells['map-proto1']) {
    if (Object.values(cell)[0].name == "typedescriptor:role0")  //we need to find user pref here for role/instance
      receptor.registerCallback(Object.values(cell)[0].name)
  }
  return new TypeDescriptorStore(receptor);
};

export const TypeDescriptorStoreProvider = {
    provide: TypeDescriptorStore,
    useFactory: typeDescriptorStoreFactory,
    deps: [HolochainService]
}

// specific holon cell - TODO inject factory with selected provider
export const HolonStoreProvider =
{ provide: HolonStore,
    useFactory: holonStoreFactory,
    deps: [HolochainService]
};

//all holon cells
export function HolonStoreProviders():Provider{
  return  { provide: HolonStore,
      useFactory: holonStoreFactory,
      deps: [HolochainService],
      multi: true
    }
}

  

@Injectable({
  providedIn:'root'
}) 
export class StoreFactory {
  private _store_dictionary: Dictionary<ComponentStore<any>|undefined> = {}
  private _selectedStore:string = ""

  constructor(private hcs:HolochainService){
    const cells = hcs.get_installed_cells()
    for (const role in cells) {
      switch(role) {  
        case "holon": this._store_dictionary[role+":"+"blah"] = undefined//serializeHash(Object.values(cells[role])[0].cell_id[1])] = undefined// new HolonStore(hcs)
      }
    }
  }

  //holon:space1 or holon:space2
  get_store(index:string){
    return this._store_dictionary[index]
  }

  getDictionarykeys():string[]{
    return Object.keys(this._store_dictionary)
  }

  setSelectedStore(key:string){
    this._selectedStore = key
  }

  getSelectedStore():ComponentStore<any>{
    //if (this._store_dictionary[this._selectedStore])
    //  return this._store_dictionary[this._selectedStore]!
    //else {
      const receptor = new HolonReceptor(this.hcs)
      receptor.registerCallback(this.getDictionarykeys()[0])// _selectedStore)
      return new HolonStore(receptor)
      //this._store_dictionary[this._selectedStore] = new HolonStore(receptor)
      //return this._store_dictionary[this._selectedStore]!
   // }
  }


  //todo dynamic add cells function
}
