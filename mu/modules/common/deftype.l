;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; deftype deftype
;;;
(core:%deftype "%deftype"
    '((:name . :symbolp)
      (:spec . :t)))

(defmacro deftype (name lambda &rest body)
  (mu:intern
   (mu:symbol-namespace)
   (mu:symbol-name)
   (common:%make-deftype name `(mu:apply (:lambda () ,@body) ())))
  `(:quote ,@name))
