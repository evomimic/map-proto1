import { Component, OnInit } from '@angular/core';

@Component({
  selector: 'app-toolbar',
  templateUrl: './toolbar.component.html'
})
export class ToolbarComponent implements OnInit {
  public openTypeList:boolean = false
  public showHolonList:boolean = false
  constructor() { }

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
