;;;  SPDX-FileCopyrightText: Copyright 2023 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; sequences
;;;
(libcore:intern :prelude "sequencep" (:lambda (t) (:if (prelude:listp t) :t (prelude:vectorp t))))

(libcore:intern :prelude "length"
   (:lambda (seq)
      (:if (prelude:sequencep seq)
           (:if (prelude:listp seq)
                (libcore:length seq)
                (libcore:sv-len seq))
           (prelude:raise seq 'prelude:length "not a sequence"))))

(libcore:intern :prelude "elt"
   (:lambda (seq nth)
      (:if (prelude:sequencep seq)
           (:if (prelude:fixnump nth)
                (:if (prelude:listp seq)
                     (libcore:nth nth seq)
                     (libcore:sv-ref seq nth))
                (prelude:raise seq 'prelude:elt "not a sequence"))
           (prelude:raise nth 'prelude:elt "not a fixnum"))))
