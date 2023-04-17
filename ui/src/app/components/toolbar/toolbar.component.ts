import { Component, Inject, OnInit } from '@angular/core';
import { HolonDescriptor, TypeHeader } from 'src/app/models/typedescriptor';
import { HolonStoreProviders, StoreFactory } from 'src/app/services/store.factory.service';
import { Observable } from 'rxjs'
import { TypeDescriptorStore } from 'src/app/stores/typedescriptor.store';
import { HolochainService } from 'src/app/services/holochain.service';

@Component({
  selector: 'app-toolbar',
  templateUrl: './toolbar.component.html',
})
export class ToolbarComponent implements OnInit {
  public openTypeList:boolean = false
  public showHolonList:boolean = false
  public TypeHeaderList$!:Observable<TypeHeader[]>
  
  constructor(private _store:TypeDescriptorStore) {
    console.log(this._store.id)
    this.TypeHeaderList$ = _store.selectTypeHeaders()
  }

  ngOnInit(): void {
  }

  toggleMenu(){
    if(this.openTypeList){
      this.openTypeList = false
      this.showHolonList = false
    }else {
      this.showHolonList = false
      this.openTypeList = true
    }
  }

  openHolonList(){
      this.openTypeList = false
      this.showHolonList = true
  }

  closeHolonList(){
    this.openTypeList = false
    this.showHolonList = false
  }

}
