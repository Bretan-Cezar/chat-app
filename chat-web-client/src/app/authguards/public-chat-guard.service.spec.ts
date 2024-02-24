import { TestBed } from '@angular/core/testing';

import { PublicChatGuard } from './public-chat-guard.service';

describe('PublicChatGuardService', () => {
  let service: PublicChatGuard;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(PublicChatGuard);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
