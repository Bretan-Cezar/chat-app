import { ComponentFixture, TestBed } from '@angular/core/testing';

import { PublicChatBoxComponent } from './public-chat-box.component';

describe('PublicChatBoxComponent', () => {
  let component: PublicChatBoxComponent;
  let fixture: ComponentFixture<PublicChatBoxComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [PublicChatBoxComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(PublicChatBoxComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
