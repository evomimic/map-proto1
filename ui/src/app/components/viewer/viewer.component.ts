import { Component, inject, Inject, OnInit } from '@angular/core';
import { Observable } from 'rxjs'; //it must use the same rxjs as the ngrx package!
import { HolonEntry } from 'src/app/models/holon';
import { HolochainService } from 'src/app/services/holochain.service';
import { HolonStoreProviders, StoreFactory } from 'src/app/services/store.factory.service';
import { HolonStore } from 'src/app/stores/holon.store';

@Component({
  selector: 'app-viewer',
  templateUrl: './viewer.component.html',
  providers: [HolonStoreProviders()]
})
export class ViewerComponent implements OnInit {
  public hnet1: string =""
  public hnet2: string =""
  public holons$! : Observable<HolonEntry[]>
  public holons2$! : Observable<HolonEntry[]>

  constructor(private readonly _stores:HolonStore) {
    console.log(_stores)
    //_stores.
  }
  
  ngOnInit(): void {
    

  }


  /*console.log(this._stores.getDictionarykeys())
    if (this._stores.getDictionarykeys().length > 0){ //entries exist
      //for (const key in this._stores.getDictionarykeys()){
      this.hnet1 = this._stores.getDictionarykeys()[0]
      this.holons$ = (this._stores.get_store(this._stores.getDictionarykeys()[0]) as HolonStore).selectHolons()
      if (this._stores.getDictionarykeys().length > 1){ //multiple networks exist
        this.hnet2 = this._stores.getDictionarykeys()[1]
        this.holons2$ = (this._stores.get_store(this._stores.getDictionarykeys()[0]) as HolonStore).selectHolons()
      }*/
}
