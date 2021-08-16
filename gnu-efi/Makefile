VERSION = 3.0.12

MKFILE_PATH := $(abspath $(lastword $(MAKEFILE_LIST)))
SRCDIR = $(dir $(MKFILE_PATH))

VPATH = $(SRCDIR)

include $(SRCDIR)/Make.defaults

SUBDIRS = lib gnuefi inc apps bootloader
gnuefi: lib

all:	check_gcc $(SUBDIRS)

mkvars:
	@echo AR=$(AR)
	@echo ARCH=$(ARCH)
	@echo ARCH3264=$(ARCH3264)
	@echo AS=$(AS)
	@echo ASFLAGS=$(ASFLAGS)
	@echo CC=$(CC)
	@echo CFLAGS=$(CFLAGS)
	@echo CPPFLAGS=$(CPPFLAGS)
	@echo GCCMINOR=$(GCCMINOR)
	@echo GCCNEWENOUGH=$(GCCNEWENOUGH)
	@echo GCCVERSION=$(GCCVERSION)
	@echo HOSTARCH=$(HOSTARCH)
	@echo INCDIR=$(INCDIR)
	@echo INSTALL=$(INSTALL)
	@echo INSTALLROOT=$(INSTALLROOT)
	@echo LD=$(LD)
	@echo LDFLAGS=$(LDFLAGS)
	@echo LIBDIR=$(LIBDIR)
	@echo OBJCOPY=$(OBJCOPY)
	@echo OS=$(OS)
	@echo prefix=$(prefix)
	@echo PREFIX=$(PREFIX)
	@echo RANLIB=$(RANLIB)
	@echo SRCDIR=$(SRCDIR)
	@echo TOPDIR=$(TOPDIR)

$(SUBDIRS):
	mkdir -p $(OBJDIR)/$@
	$(MAKE) -C $(OBJDIR)/$@ -f $(SRCDIR)/$@/Makefile SRCDIR=$(SRCDIR)/$@ ARCH=$(ARCH)

clean:
	rm -f *~
	@for d in $(SUBDIRS); do \
		if [ -d $(OBJDIR)/$$d ]; then \
			$(MAKE) -C $(OBJDIR)/$$d -f $(SRCDIR)/$$d/Makefile SRCDIR=$(SRCDIR)/$$d clean; \
		fi; \
	done

install:
	@for d in $(SUBDIRS); do \
		mkdir -p $(OBJDIR)/$$d; \
		$(MAKE) -C $(OBJDIR)/$$d -f $(SRCDIR)/$$d/Makefile SRCDIR=$(SRCDIR)/$$d install; done

.PHONY:	$(SUBDIRS) clean depend

#
# on both platforms you must use gcc 3.0 or higher 
#
check_gcc:
ifeq ($(GCC_VERSION),2)
	@echo "you need to use a version of gcc >= 3.0, you are using `$(CC) --version`"
	@exit 1
endif

include $(SRCDIR)/Make.rules

test-archive:
	@rm -rf /tmp/gnu-efi-$(VERSION) /tmp/gnu-efi-$(VERSION)-tmp
	@mkdir -p /tmp/gnu-efi-$(VERSION)-tmp
	@git archive --format=tar $(shell git branch | awk '/^*/ { print $$2 }') | ( cd /tmp/gnu-efi-$(VERSION)-tmp/ ; tar x )
	@git diff | ( cd /tmp/gnu-efi-$(VERSION)-tmp/ ; patch -s -p1 -b -z .gitdiff )
	@mv /tmp/gnu-efi-$(VERSION)-tmp/ /tmp/gnu-efi-$(VERSION)/
	@dir=$$PWD; cd /tmp; tar -c --bzip2 -f $$dir/gnu-efi-$(VERSION).tar.bz2 gnu-efi-$(VERSION)
	@rm -rf /tmp/gnu-efi-$(VERSION)
	@echo "The archive is in gnu-efi-$(VERSION).tar.bz2"

tag:
	git tag $(VERSION) refs/heads/master

archive: tag
	@rm -rf /tmp/gnu-efi-$(VERSION) /tmp/gnu-efi-$(VERSION)-tmp
	@mkdir -p /tmp/gnu-efi-$(VERSION)-tmp
	@git archive --format=tar $(VERSION) | ( cd /tmp/gnu-efi-$(VERSION)-tmp/ ; tar x )
	@mv /tmp/gnu-efi-$(VERSION)-tmp/ /tmp/gnu-efi-$(VERSION)/
	@dir=$$PWD; cd /tmp; tar -c --bzip2 -f $$dir/gnu-efi-$(VERSION).tar.bz2 gnu-efi-$(VERSION)
	@rm -rf /tmp/gnu-efi-$(VERSION)
	@echo "The archive is in gnu-efi-$(VERSION).tar.bz2"

