import { ComponentFixture, TestBed } from '@angular/core/testing';

import { PublicChatNavBarComponent } from './public-chat-nav-bar.component';

describe('NavBarComponent', () => {
  let component: PublicChatNavBarComponent;
  let fixture: ComponentFixture<PublicChatNavBarComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [PublicChatNavBarComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(PublicChatNavBarComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
